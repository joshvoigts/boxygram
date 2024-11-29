# Boxygram

A web server for collaborative drawing.

1. Go to the site index
2. Click "new"
3. Share the link to the drawing box with someone else


## Setup

    git clone https://github.com/joshvoigts/boxygram.git
    cd boxygram
    cat ddl/init.sql | sqlite3 drawings.db

    # Optionally, set config vars by creating a `.env` file:
    BOXY_BIND_ADDRESS="0.0.0.0"
    BOXY_BIND_PORT=80

    make serve
