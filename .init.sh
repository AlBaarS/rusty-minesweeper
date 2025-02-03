sudo systemctl start mongod

(trap 'kill 0' SIGINT; (npm run build) & (cd backend/ ; cargo build --release))

sudo systemctl stop mongod
