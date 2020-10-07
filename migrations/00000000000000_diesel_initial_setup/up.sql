-- This file was automatically created by Diesel to setup helper functions
-- and other internal bookkeeping. This file is safe to edit, any future
-- changes will be added to existing projects as new migrations.


CREATE SCHEMA pauth;
create user pauth_admin with password 'pauth_admin_password';

ALTER SCHEMA pauth OWNER TO postgres;

--
-- Name: SCHEMA pauth; Type: COMMENT; Schema: -; Owner: postgres
--

COMMENT ON SCHEMA pauth IS 'standard pauth schema';


--
-- Name: plpgsql; Type: EXTENSION; Schema: -; Owner:
--

CREATE EXTENSION IF NOT EXISTS plpgsql WITH SCHEMA pg_catalog;


--
-- Name: EXTENSION plpgsql; Type: COMMENT; Schema: -; Owner:
--

COMMENT ON EXTENSION plpgsql IS 'PL/pgSQL procedural language';


--
-- Name: pgcrypto; Type: EXTENSION; Schema: -; Owner:
--

CREATE EXTENSION IF NOT EXISTS pgcrypto WITH SCHEMA pauth;


--
-- Name: EXTENSION pgcrypto; Type: COMMENT; Schema: -; Owner:
--

COMMENT ON EXTENSION pgcrypto IS 'cryptographic functions';

SET default_tablespace = '';

SET default_with_oids = false;


CREATE TABLE pauth.pw_reset (
                                id integer NOT NULL,
                                user_id integer NOT NULL,
                                user_token_hash text NOT NULL,
                                expires timestamp without time zone
);


ALTER TABLE pauth.pw_reset OWNER TO pauth_admin;

--
-- Name: pw_reset_id_seq; Type: SEQUENCE; Schema: pauth; Owner: pauth_admin
--

CREATE SEQUENCE pauth.pw_reset_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE pauth.pw_reset_id_seq OWNER TO pauth_admin;

--
-- Name: pw_reset_id_seq; Type: SEQUENCE OWNED BY; Schema: pauth; Owner: pauth_admin
--

ALTER SEQUENCE pauth.pw_reset_id_seq OWNED BY pauth.pw_reset.id;


--
-- Name: user_login_tokens; Type: TABLE; Schema: pauth; Owner: pauth_admin
--

CREATE TABLE pauth.user_login_tokens (
                                         id integer NOT NULL,
                                         user_id integer NOT NULL,
                                         token text NOT NULL,
                                         created timestamp without time zone DEFAULT now() NOT NULL,
                                         last_used timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE pauth.user_login_tokens OWNER TO pauth_admin;

--
-- Name: user_login_tokens_id_seq; Type: SEQUENCE; Schema: pauth; Owner: pauth_admin
--

CREATE SEQUENCE pauth.user_login_tokens_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE pauth.user_login_tokens_id_seq OWNER TO pauth_admin;

--
-- Name: user_login_tokens_id_seq; Type: SEQUENCE OWNED BY; Schema: pauth; Owner: pauth_admin
--

ALTER SEQUENCE pauth.user_login_tokens_id_seq OWNED BY pauth.user_login_tokens.id;


--
-- Name: users; Type: TABLE; Schema: pauth; Owner: pauth_admin
--

CREATE TABLE pauth.users (
                             id integer NOT NULL,
                             chosen_name character varying NOT NULL,
                             email character varying NOT NULL,
                             pass_hash text NOT NULL,
                             last_login timestamp without time zone DEFAULT now() NOT NULL
);


ALTER TABLE pauth.users OWNER TO pauth_admin;

--
-- Name: users_id_seq; Type: SEQUENCE; Schema: pauth; Owner: pauth_admin
--

CREATE SEQUENCE pauth.users_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE pauth.users_id_seq OWNER TO pauth_admin;

--
-- Name: users_id_seq; Type: SEQUENCE OWNED BY; Schema: pauth; Owner: pauth_admin
--

ALTER SEQUENCE pauth.users_id_seq OWNED BY pauth.users.id;

--
-- Name: id; Type: DEFAULT; Schema: pauth; Owner: pauth_admin
--

ALTER TABLE ONLY pauth.pw_reset ALTER COLUMN id SET DEFAULT nextval('pauth.pw_reset_id_seq'::regclass);


--
-- Name: id; Type: DEFAULT; Schema: pauth; Owner: pauth_admin
--


ALTER TABLE ONLY pauth.user_login_tokens ALTER COLUMN id SET DEFAULT nextval('pauth.user_login_tokens_id_seq'::regclass);


--
-- Name: id; Type: DEFAULT; Schema: pauth; Owner: pauth_admin
--

ALTER TABLE ONLY pauth.users ALTER COLUMN id SET DEFAULT nextval('pauth.users_id_seq'::regclass);


--
-- Name: pw_reset_pkey; Type: CONSTRAINT; Schema: pauth; Owner: pauth_admin
--

ALTER TABLE ONLY pauth.pw_reset
    ADD CONSTRAINT pw_reset_pkey PRIMARY KEY (id);


--
-- Name: user_login_tokens_pkey; Type: CONSTRAINT; Schema: pauth; Owner: pauth_admin
--

ALTER TABLE ONLY pauth.user_login_tokens
    ADD CONSTRAINT user_login_tokens_pkey PRIMARY KEY (id);


--
-- Name: users_pkey; Type: CONSTRAINT; Schema: pauth; Owner: pauth_admin
--

ALTER TABLE ONLY pauth.users
    ADD CONSTRAINT users_pkey PRIMARY KEY (id);


--
-- Name: pw_reset_user_id_fkey; Type: INDEX; Schema: pauth; Owner: pauth_admin
--

CREATE INDEX pw_reset_user_id_fkey ON pauth.pw_reset USING btree (user_id);


--
-- Name: user_login_tokens_user_id_fkey; Type: INDEX; Schema: pauth; Owner: pauth_admin
--

CREATE INDEX user_login_tokens_user_id_fkey ON pauth.user_login_tokens USING btree (user_id);


--
-- Name: users_chosen_name_idx; Type: INDEX; Schema: pauth; Owner: pauth_admin
--

CREATE INDEX users_chosen_name_idx ON pauth.users USING btree (chosen_name);


--
-- Name: users_email_idx; Type: INDEX; Schema: pauth; Owner: pauth_admin
--

CREATE INDEX users_email_idx ON pauth.users USING btree (email);

--
-- Name: pw_reset_user_id_fkey; Type: FK CONSTRAINT; Schema: pauth; Owner: pauth_admin
--

ALTER TABLE ONLY pauth.pw_reset
    ADD CONSTRAINT pw_reset_user_id_fkey FOREIGN KEY (user_id) REFERENCES pauth.users(id) ON DELETE CASCADE;


--
-- Name: user_login_tokens_user_id_fkey; Type: FK CONSTRAINT; Schema: pauth; Owner: pauth_admin
--

ALTER TABLE ONLY pauth.user_login_tokens
    ADD CONSTRAINT user_login_tokens_user_id_fkey FOREIGN KEY (user_id) REFERENCES pauth.users(id) ON DELETE CASCADE;


grant all on schema pauth to pauth_admin;

-- Sets up a trigger for the given table to automatically set a column called
-- `updated_at` whenever the row is modified (unless `updated_at` was included
-- in the modified columns)
--
-- # Example
--
-- ```sql
-- CREATE TABLE users (id SERIAL PRIMARY KEY, updated_at TIMESTAMP NOT NULL DEFAULT NOW());
--
-- SELECT diesel_manage_updated_at('users');
-- ```
CREATE OR REPLACE FUNCTION diesel_manage_updated_at(_tbl regclass) RETURNS VOID AS $$
BEGIN
    EXECUTE format('CREATE TRIGGER set_updated_at BEFORE UPDATE ON %s
                    FOR EACH ROW EXECUTE PROCEDURE diesel_set_updated_at()', _tbl);
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION diesel_set_updated_at() RETURNS trigger AS $$
BEGIN
    IF (
        NEW IS DISTINCT FROM OLD AND
        NEW.updated_at IS NOT DISTINCT FROM OLD.updated_at
    ) THEN
        NEW.updated_at := current_timestamp;
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
