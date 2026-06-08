FROM postgres:15

ENV POSTGRES_USER=postgres
ENV POSTGRES_PASSWORD=postgres
ENV POSTGRES_DB=app_db

# Expose the default Postgres port
EXPOSE 123


# docker run --name my-postgres --env-file .env -p 5432:5432 -d postgres:15
