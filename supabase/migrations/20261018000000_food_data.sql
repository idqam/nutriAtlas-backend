CREATE TABLE public.food_category (
    id BIGINT PRIMARY KEY,
    code TEXT,
    description TEXT NOT NULL
);

CREATE TABLE public.nutrient (
    id BIGINT PRIMARY KEY,
    name TEXT NOT NULL,
    unit_name TEXT,
    nutrient_nbr NUMERIC,
    rank NUMERIC
);

CREATE TABLE public.measure_unit (
    id BIGINT PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE public.food_attribute_type (
    id BIGINT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT
);

CREATE TABLE public.food (
    fdc_id BIGINT PRIMARY KEY,
    data_type TEXT NOT NULL,
    description TEXT,
    food_category_id BIGINT REFERENCES public.food_category(id),
    publication_date DATE,
    market_country TEXT,
    trade_channel TEXT
);

CREATE TABLE public.branded_food (
    fdc_id BIGINT PRIMARY KEY REFERENCES public.food(fdc_id) ON DELETE CASCADE,
    brand_owner TEXT,
    brand_name TEXT,
    subbrand_name TEXT,
    gtin_upc TEXT,
    ingredients TEXT,
    not_a_significant_source_of TEXT,
    serving_size NUMERIC,
    serving_size_unit TEXT,
    household_serving_fulltext TEXT,
    branded_food_category TEXT,
    data_source TEXT,
    package_weight TEXT,
    modified_date DATE,
    available_date DATE,
    market_country TEXT,
    discontinued_date DATE,
    preparation_state_code TEXT,
    trade_channel TEXT,
    short_description TEXT,
    material_code TEXT
);

CREATE TABLE public.food_attribute (
    id BIGINT PRIMARY KEY,
    fdc_id BIGINT NOT NULL REFERENCES public.food(fdc_id) ON DELETE CASCADE,
    seq_num INTEGER,
    food_attribute_type_id BIGINT REFERENCES public.food_attribute_type(id),
    name TEXT,
    value TEXT
);

CREATE TABLE public.food_nutrient (
    id BIGINT PRIMARY KEY,
    fdc_id BIGINT NOT NULL REFERENCES public.food(fdc_id) ON DELETE CASCADE,
    nutrient_id BIGINT NOT NULL REFERENCES public.nutrient(id),
    amount NUMERIC,
    data_points INTEGER,
    derivation_id INTEGER,
    min NUMERIC,
    max NUMERIC,
    median NUMERIC,
    footnote TEXT,
    min_year_acquired INTEGER
);

CREATE TABLE public.food_portion (
    id BIGINT PRIMARY KEY,
    fdc_id BIGINT NOT NULL REFERENCES public.food(fdc_id) ON DELETE CASCADE,
    seq_num INTEGER,
    amount NUMERIC,
    measure_unit_id BIGINT REFERENCES public.measure_unit(id),
    portion_description TEXT,
    modifier TEXT,
    gram_weight NUMERIC,
    data_points INTEGER,
    footnote TEXT,
    min_year_acquired INTEGER
);

CREATE TABLE public.input_food (
    id BIGINT PRIMARY KEY,
    fdc_id BIGINT NOT NULL REFERENCES public.food(fdc_id) ON DELETE CASCADE,
    fdc_of_input_food BIGINT REFERENCES public.food(fdc_id),
    seq_num INTEGER,
    amount NUMERIC,
    ingredient_code TEXT,
    ingredient_description TEXT,
    unit TEXT,
    portion_code TEXT,
    portion_description TEXT,
    gram_weight NUMERIC,
    retention_code TEXT
);

CREATE INDEX food_category_description_idx ON public.food_category (description);
CREATE INDEX food_food_category_id_idx ON public.food (food_category_id);
CREATE INDEX food_attribute_fdc_id_idx ON public.food_attribute (fdc_id);
CREATE INDEX food_nutrient_fdc_nutrient_idx ON public.food_nutrient (fdc_id, nutrient_id);
CREATE INDEX food_portion_fdc_id_idx ON public.food_portion (fdc_id);
CREATE INDEX input_food_fdc_id_idx ON public.input_food (fdc_id);

CREATE MATERIALIZED VIEW public.food_search AS
SELECT
    f.fdc_id,
    f.description,
    f.data_type,
    f.food_category_id,
    fc.description AS food_category,
    bf.brand_owner,
    bf.brand_name,
    bf.subbrand_name,
    bf.gtin_upc,
    bf.ingredients,
    bf.serving_size,
    bf.serving_size_unit,
    bf.household_serving_fulltext,
    COALESCE(attrs.aliases, '') AS aliases,
    setweight(to_tsvector('english', COALESCE(f.description, '')), 'A') ||
    setweight(
        to_tsvector(
            'english',
            concat_ws(' ', fc.description, bf.brand_owner, bf.brand_name,
                      bf.subbrand_name, attrs.aliases)
        ),
        'B'
    ) ||
    setweight(to_tsvector('english', COALESCE(bf.ingredients, '')), 'C') AS search_vector
FROM public.food AS f
LEFT JOIN public.food_category AS fc ON fc.id = f.food_category_id
LEFT JOIN public.branded_food AS bf ON bf.fdc_id = f.fdc_id
LEFT JOIN LATERAL (
    SELECT string_agg(a.value, ' ') AS aliases
    FROM public.food_attribute AS a
    WHERE a.fdc_id = f.fdc_id
      AND lower(a.name) IN ('common name', 'additional description')
) AS attrs ON TRUE;

CREATE UNIQUE INDEX food_search_fdc_id_idx ON public.food_search (fdc_id);
CREATE INDEX food_search_vector_idx ON public.food_search USING GIN (search_vector);
CREATE INDEX food_search_gtin_upc_idx ON public.food_search (gtin_upc) WHERE gtin_upc IS NOT NULL;
CREATE INDEX food_search_name_idx ON public.food_search (lower(description) text_pattern_ops);