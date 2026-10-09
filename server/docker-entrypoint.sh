#!/bin/sh
# Runs the three ACEmu servers in one container and stops when any of them
# exits, so the container restarts as a unit.
set -u
cd /app

if [ "${ACEMU_PUBLIC_IP:-127.0.0.1}" = "127.0.0.1" ]; then
  echo "WARNING: ACEMU_PUBLIC_IP is 127.0.0.1 - clients on other machines will be sent to themselves. Set it to this host's LAN/public IP." >&2
fi

./acemu-login &
LOGIN=$!
./acemu-channel &
CHANNEL=$!
./acemu-admin &
ADMIN=$!

stop() {
  kill "$LOGIN" "$CHANNEL" "$ADMIN" 2>/dev/null
}
trap stop INT TERM

# `wait -n` is bash-only; poll instead (busybox/dash sh).
while kill -0 "$LOGIN" 2>/dev/null && kill -0 "$CHANNEL" 2>/dev/null && kill -0 "$ADMIN" 2>/dev/null; do
  sleep 2
done
echo "a server exited - stopping the others" >&2
stop
wait
exit 1
