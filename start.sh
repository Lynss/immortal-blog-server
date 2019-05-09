#!/usr/bin/env bash

systemfd --no-pid -s http::8083 -- cargo watch -x run -i logs/* -package backend;
