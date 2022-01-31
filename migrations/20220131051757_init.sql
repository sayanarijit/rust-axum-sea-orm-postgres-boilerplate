CREATE TABLE public.events (
	id uuid NOT NULL,
	title text NOT NULL,
	short_description text NOT NULL,
	long_description text NOT NULL,
	thumbnail text,
	cover text NOT NULL,
	budget integer,
	CONSTRAINT events_pk PRIMARY KEY (id)
);
-- ddl-end --
ALTER TABLE public.events OWNER TO postgres;
