ALTER TABLE public.user_profile
    ADD COLUMN goals TEXT[] NOT NULL DEFAULT ARRAY['non_specified']::TEXT[]
    CHECK (
        cardinality(goals) > 0
        AND goals <@ ARRAY[
            'lose_weight',
            'better_health',
            'gain_weight',
            'non_specified'
        ]::TEXT[]
        AND ('non_specified' <> ALL(goals) OR cardinality(goals) = 1)
        AND NOT (
            'lose_weight' = ANY(goals)
            AND 'gain_weight' = ANY(goals)
        )
    );

CREATE TABLE public.recipe (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_user_id UUID NOT NULL REFERENCES public.app_user(user_id) ON DELETE CASCADE,
    name TEXT NOT NULL CHECK (char_length(btrim(name)) BETWEEN 1 AND 200),
    description TEXT,
    instructions TEXT,
    total_servings NUMERIC(8, 2) NOT NULL DEFAULT 1
        CHECK (total_servings > 0 AND total_servings < 'Infinity'::NUMERIC),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE public.recipe_item (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    recipe_id UUID NOT NULL REFERENCES public.recipe(id) ON DELETE CASCADE,
    position INTEGER NOT NULL CHECK (position >= 0),
    fdc_id BIGINT REFERENCES public.food(fdc_id) ON DELETE RESTRICT,
    custom_food_id UUID REFERENCES public.custom_food(id) ON DELETE RESTRICT,
    source TEXT NOT NULL CHECK (source IN ('common', 'branded', 'custom')),
    food_name TEXT NOT NULL CHECK (char_length(btrim(food_name)) BETWEEN 1 AND 200),
    quantity NUMERIC(12, 4) NOT NULL
        CHECK (quantity > 0 AND quantity < 'Infinity'::NUMERIC),
    unit TEXT NOT NULL CHECK (char_length(btrim(unit)) > 0),
    grams NUMERIC(12, 3) NOT NULL
        CHECK (grams > 0 AND grams < 'Infinity'::NUMERIC),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (recipe_id, position),
    CONSTRAINT recipe_item_source_reference_check CHECK (
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
    )
);

CREATE INDEX recipe_owner_updated_at_idx
    ON public.recipe (owner_user_id, updated_at DESC);
CREATE INDEX recipe_item_fdc_id_idx
    ON public.recipe_item (fdc_id)
    WHERE fdc_id IS NOT NULL;
CREATE INDEX recipe_item_custom_food_id_idx
    ON public.recipe_item (custom_food_id)
    WHERE custom_food_id IS NOT NULL;

ALTER TABLE public.recipe ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.recipe_item ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Users can manage their recipes"
    ON public.recipe FOR ALL TO authenticated
    USING ((SELECT auth.uid()) = owner_user_id)
    WITH CHECK ((SELECT auth.uid()) = owner_user_id);

CREATE POLICY "Users can read items from their recipes"
    ON public.recipe_item FOR SELECT TO authenticated
    USING (
        EXISTS (
            SELECT 1
            FROM public.recipe AS r
            WHERE r.id = recipe_id
              AND r.owner_user_id = (SELECT auth.uid())
        )
    );

CREATE POLICY "Users can add items to their recipes"
    ON public.recipe_item FOR INSERT TO authenticated
    WITH CHECK (
        EXISTS (
            SELECT 1
            FROM public.recipe AS r
            WHERE r.id = recipe_id
              AND r.owner_user_id = (SELECT auth.uid())
        )
        AND (
            source IN ('common', 'branded')
            OR EXISTS (
                SELECT 1
                FROM public.custom_food AS cf
                WHERE cf.id = custom_food_id
                  AND cf.owner_user_id = (SELECT auth.uid())
            )
        )
    );

CREATE POLICY "Users can update items in their recipes"
    ON public.recipe_item FOR UPDATE TO authenticated
    USING (
        EXISTS (
            SELECT 1
            FROM public.recipe AS r
            WHERE r.id = recipe_id
              AND r.owner_user_id = (SELECT auth.uid())
        )
    )
    WITH CHECK (
        EXISTS (
            SELECT 1
            FROM public.recipe AS r
            WHERE r.id = recipe_id
              AND r.owner_user_id = (SELECT auth.uid())
        )
        AND (
            source IN ('common', 'branded')
            OR EXISTS (
                SELECT 1
                FROM public.custom_food AS cf
                WHERE cf.id = custom_food_id
                  AND cf.owner_user_id = (SELECT auth.uid())
            )
        )
    );

CREATE POLICY "Users can delete items from their recipes"
    ON public.recipe_item FOR DELETE TO authenticated
    USING (
        EXISTS (
            SELECT 1
            FROM public.recipe AS r
            WHERE r.id = recipe_id
              AND r.owner_user_id = (SELECT auth.uid())
        )
    );