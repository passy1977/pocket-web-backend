#!/bin/bash

 cargo build --release


if [ -f pocket-web-backend ]; then
    rm pocket-web-backend
fi

 cp target/release/pocket-web-backend .

./pocket-web-backend