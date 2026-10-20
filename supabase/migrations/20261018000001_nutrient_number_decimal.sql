ALTER TABLE public.nutrient
    ALTER COLUMN nutrient_nbr TYPE NUMERIC
    USING nutrient_nbr::NUMERIC;