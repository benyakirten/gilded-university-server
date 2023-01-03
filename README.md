# What is this?

It's a backend for the Gilded University app (maybe frontend too). It uses Diesel with a Postgres database and graphql.
This readme will be updated when the project is further along.

## Required Setup
**TODO**

### Required Environment Variables
A `.env` file should be created in the top level folder with the following valus:

* `DATABASE_URL` - this should be a connection URL to connect to a postgres database. It must include the database name at the end.
* `JWT_SECRET` - a secret used to encode and decode JWTs
* `TEST_DATABASE_URL` - the URL for the database used for tests in the tests folder (integration and e2e tests)