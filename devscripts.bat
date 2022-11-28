@REM Watch for changes and perform checks, tests, and run
cargo watch -x check -x test -x run

@REM Watch for changes and perform checks and run
cargo watch -x check -x run

@REM Check the IP address postgres is running on in docker 
@REM to know where to point server in pgadmin
cat /etc/hosts