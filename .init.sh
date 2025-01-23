sudo systemctl start mongod

(trap 'kill 0' SIGINT; (npm run dev) & (cd backend/ ; cargo run))

sudo systemctl stop mongod
