CREATE TABLE public.app_user (
    user_id UUID PRIMARY KEY REFERENCES auth.users(id) ON DELETE CASCADE,
    role TEXT NOT NULL DEFAULT 'user'
        CHECK (role IN ('user', 'coach', 'admin')),
    plan TEXT NOT NULL DEFAULT 'free'
        CHECK (plan IN ('free', 'premium')),
    timezone TEXT NOT NULL DEFAULT 'UTC',
    pref_units TEXT NOT NULL DEFAULT 'metric'
        CHECK (pref_units IN ('metric', 'us')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE public.user_profile (
    user_id UUID PRIMARY KEY REFERENCES public.app_user(user_id) ON DELETE CASCADE,
    sex TEXT CHECK (sex IN ('female', 'male', 'intersex')),
    birth_date DATE CHECK (birth_date IS NULL OR birth_date <= CURRENT_DATE),
    height_cm NUMERIC(6, 2)
        CHECK (height_cm IS NULL OR (height_cm > 0 AND height_cm < 'Infinity'::NUMERIC)),
    current_weight_kg NUMERIC(7, 3)
        CHECK (
            current_weight_kg IS NULL
            OR (current_weight_kg > 0 AND current_weight_kg < 'Infinity'::NUMERIC)
        ),
    activity_level TEXT CHECK (
        activity_level IS NULL OR activity_level IN (
            'sedentary',
            'lightly_active',
            'moderately_active',
            'very_active',
            'extremely_active'
        )
    ),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE public.daily_targets (
    user_id UUID NOT NULL REFERENCES public.app_user(user_id) ON DELETE CASCADE,
    effective_from DATE NOT NULL,
    calories_kcal NUMERIC(8, 2) NOT NULL
        CHECK (calories_kcal > 0 AND calories_kcal < 'Infinity'::NUMERIC),
    protein_g NUMERIC(8, 2) NOT NULL
        CHECK (protein_g >= 0 AND protein_g < 'Infinity'::NUMERIC),
    carbs_g NUMERIC(8, 2) NOT NULL
        CHECK (carbs_g >= 0 AND carbs_g < 'Infinity'::NUMERIC),
    fat_g NUMERIC(8, 2) NOT NULL
        CHECK (fat_g >= 0 AND fat_g < 'Infinity'::NUMERIC),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (user_id, effective_from)
);

CREATE TABLE public.weight_entry (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES public.app_user(user_id) ON DELETE CASCADE,
    kg DOUBLE PRECISION NOT NULL CHECK (kg > 0 AND kg < 'Infinity'::DOUBLE PRECISION),
    measured_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE public.custom_food (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_user_id UUID NOT NULL REFERENCES public.app_user(user_id) ON DELETE CASCADE,
    name TEXT NOT NULL CHECK (char_length(btrim(name)) BETWEEN 1 AND 200),
    brand TEXT,
    nutrient_profile_per_100g JSONB NOT NULL DEFAULT '{}'::JSONB
        CHECK (jsonb_typeof(nutrient_profile_per_100g) = 'object'),
    servings JSONB NOT NULL DEFAULT '[]'::JSONB
        CHECK (jsonb_typeof(servings) = 'array'),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX daily_targets_effective_date_idx
    ON public.daily_targets (user_id, effective_from DESC);
CREATE INDEX weight_entry_user_measured_at_idx
    ON public.weight_entry (user_id, measured_at DESC);
CREATE INDEX custom_food_owner_created_at_idx
    ON public.custom_food (owner_user_id, created_at DESC);

ALTER TABLE public.app_user ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.user_profile ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.daily_targets ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.weight_entry ENABLE ROW LEVEL SECURITY;
ALTER TABLE public.custom_food ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Users can read their app account"
    ON public.app_user FOR SELECT TO authenticated
    USING ((SELECT auth.uid()) = user_id);

CREATE POLICY "Users can manage their profile"
    ON public.user_profile FOR ALL TO authenticated
    USING ((SELECT auth.uid()) = user_id)
    WITH CHECK ((SELECT auth.uid()) = user_id);

CREATE POLICY "Users can manage their daily targets"
    ON public.daily_targets FOR ALL TO authenticated
    USING ((SELECT auth.uid()) = user_id)
    WITH CHECK ((SELECT auth.uid()) = user_id);

CREATE POLICY "Users can manage their weight entries"
    ON public.weight_entry FOR ALL TO authenticated
    USING ((SELECT auth.uid()) = user_id)
    WITH CHECK ((SELECT auth.uid()) = user_id);

CREATE POLICY "Users can manage their custom foods"
    ON public.custom_food FOR ALL TO authenticated
    USING ((SELECT auth.uid()) = owner_user_id)
    WITH CHECK ((SELECT auth.uid()) = owner_user_id);

CREATE FUNCTION public.handle_new_auth_user()
RETURNS TRIGGER
LANGUAGE plpgsql
SECURITY DEFINER
SET search_path = ''
AS $$
BEGIN
    INSERT INTO public.app_user (user_id)
    VALUES (NEW.id);

    INSERT INTO public.user_profile (user_id)
    VALUES (NEW.id);

    RETURN NEW;
END;
$$;

CREATE TRIGGER on_auth_user_created
    AFTER INSERT ON auth.users
    FOR EACH ROW EXECUTE FUNCTION public.handle_new_auth_user();