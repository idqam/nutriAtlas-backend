CREATE TABLE public.food_log_entry (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES public.app_user(user_id) ON DELETE CASCADE,
    fdc_id BIGINT REFERENCES public.food(fdc_id) ON DELETE RESTRICT,
    custom_food_id UUID REFERENCES public.custom_food(id) ON DELETE RESTRICT,
    source TEXT NOT NULL CHECK (source IN ('common', 'branded', 'custom', 'quick_add')),
    food_name TEXT NOT NULL CHECK (char_length(btrim(food_name)) BETWEEN 1 AND 200),
    brand TEXT,
    unit TEXT NOT NULL CHECK (char_length(btrim(unit)) > 0),
    quantity NUMERIC(12, 4) NOT NULL
        CHECK (quantity > 0 AND quantity < 'Infinity'::NUMERIC),
    grams NUMERIC(12, 3) NOT NULL
        CHECK (grams > 0 AND grams < 'Infinity'::NUMERIC),
    nutrients JSONB NOT NULL CHECK (jsonb_typeof(nutrients) = 'object'),
    meal_type TEXT NOT NULL
        CHECK (meal_type IN ('breakfast', 'lunch', 'dinner', 'snack')),
    consumed_at TIMESTAMPTZ NOT NULL,
    local_date DATE NOT NULL,
    revision BIGINT NOT NULL DEFAULT 1 CHECK (revision > 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT food_log_entry_source_reference_check CHECK (
        (
            source IN ('common', 'branded')
            AND fdc_id IS NOT NULL
            AND custom_food_id IS NULL
        )
        OR (
            source = 'custom'
            AND fdc_id IS NULL
            AND custom_food_id IS NOT NULL
        )
        OR (
            source = 'quick_add'
            AND fdc_id IS NULL
            AND custom_food_id IS NULL
        )
    )
);

CREATE TABLE public.outbox_event (
    event_id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES public.app_user(user_id) ON DELETE CASCADE,
    event_type TEXT NOT NULL CHECK (char_length(btrim(event_type)) > 0),
    schema_version SMALLINT NOT NULL CHECK (schema_version > 0),
    payload JSONB NOT NULL CHECK (jsonb_typeof(payload) = 'object'),
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    available_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    published_at TIMESTAMPTZ,
    attempts INTEGER NOT NULL DEFAULT 0 CHECK (attempts >= 0),
    locked_until TIMESTAMPTZ,
    lock_token UUID,
    last_error TEXT,
    CONSTRAINT outbox_event_lock_pair_check CHECK (
        (locked_until IS NULL) = (lock_token IS NULL)
    ),
    CONSTRAINT outbox_event_published_unlocked_check CHECK (
        published_at IS NULL OR (locked_until IS NULL AND lock_token IS NULL)
    )
);

CREATE INDEX food_log_entry_user_local_date_idx
    ON public.food_log_entry (user_id, local_date, consumed_at);
CREATE INDEX food_log_entry_fdc_id_idx
    ON public.food_log_entry (fdc_id)
    WHERE fdc_id IS NOT NULL;
CREATE INDEX food_log_entry_custom_food_id_idx
    ON public.food_log_entry (custom_food_id)
    WHERE custom_food_id IS NOT NULL;
CREATE INDEX outbox_event_pending_idx
    ON public.outbox_event (available_at, created_at)
    WHERE published_at IS NULL;

ALTER TABLE public.food_log_entry ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.outbox_event ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Users can read their food log entries"
    ON public.food_log_entry FOR SELECT TO authenticated
    USING ((SELECT auth.uid()) = user_id);

CREATE POLICY "Users can create food log entries for their own foods"
    ON public.food_log_entry FOR INSERT TO authenticated
    WITH CHECK (
        (SELECT auth.uid()) = user_id
        AND (
            source IN ('common', 'branded', 'quick_add')
            OR EXISTS (
                SELECT 1
                FROM public.custom_food AS cf
                WHERE cf.id = custom_food_id
                  AND cf.owner_user_id = (SELECT auth.uid())
            )
        )
    );

CREATE POLICY "Users can update their food log entries"
    ON public.food_log_entry FOR UPDATE TO authenticated
    USING ((SELECT auth.uid()) = user_id)
    WITH CHECK (
        (SELECT auth.uid()) = user_id
        AND (
            source IN ('common', 'branded', 'quick_add')
            OR EXISTS (
                SELECT 1
                FROM public.custom_food AS cf
                WHERE cf.id = custom_food_id
                  AND cf.owner_user_id = (SELECT auth.uid())
            )
        )
    );

CREATE POLICY "Users can delete their food log entries"
    ON public.food_log_entry FOR DELETE TO authenticated
    USING ((SELECT auth.uid()) = user_id);