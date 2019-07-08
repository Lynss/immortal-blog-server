#!/usr/bin/env bash

systemfd --no-pid -s http::8083 -- cargo watch -x run -i logs/* -i attr.txt -i input.txt -i static/* -package backend;
