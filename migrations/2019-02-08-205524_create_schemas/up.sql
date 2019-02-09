CREATE SCHEMA app_public;
CREATE SCHEMA app_hidden;
CREATE SCHEMA app_private;

COMMENT ON SCHEMA app_public  IS 'Tables and functions to be exposed to the public API.';
COMMENT ON SCHEMA app_hidden  IS 'Same privileges as app_public, but simply not exposed directly to the API.';
COMMENT ON SCHEMA app_private IS 'Tables and functions that require elevated privileges to access.';
