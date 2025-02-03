systemctl start mongod

(trap 'kill 0' SIGINT; (npm run build) & (cd backend/ ; cargo build --release))

systemctl stop mongod
