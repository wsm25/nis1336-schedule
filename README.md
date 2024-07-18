# Schedule Manager

The SJTU NIS1336 coursework.

## Development Deployment
Run `yarn dev` in `/frontend` and `cargo run` in `/backend`.

## Production Deployment
> [!CAUTION]
> The programme is really unstable and may contain tons of bugs.
> Do not deploy it to production environment.

First put frontend product to `/var/www/schedule` and backend server
executive to `/usr/local/bin`, and mkdir `/var/lib/private/schedule/db`

Systemd configuration:
`/lib/systemd/system/nis1336-schedule.service`
```ini
[Unit]
Description=nis1336 schedule
Documentation=https://github.com/wsm25/nis1336-schedule
After=network.target

[Service]
DynamicUser=true
Environment=SCHEDULE_PORT=8080
WorkingDirectory=/var/lib/schedule
StateDirectory=schedule
ExecStart=/usr/local/bin/schedule-server
Restart=on-abnormal
RestartSec=5s
KillMode=mixed

StandardOutput=journal
StandardError=journal

[Install]
WantedBy=multi-user.target
```

Nginx configuration:
`/etc/nginx/sites-enabled/nis1336-schedule`
```nginx
server {
    listen 8000;
    server_name YOUR_SERVER_NAME;
    location ~ ^/(api|auth) {
            proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
            proxy_set_header Host $http_host;
            proxy_redirect off;
            proxy_pass http://[::1]:8080;
            client_max_body_size 200m;
    }
    location / {
            root /var/www/schedule/;
    }
}
```

And then `nginx -s reload && systemctl start nis1336-schedule`