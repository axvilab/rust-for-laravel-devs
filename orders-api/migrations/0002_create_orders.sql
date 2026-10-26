CREATE TYPE order_status AS ENUM ('draft', 'paid', 'cancelled');

CREATE TABLE orders (
    id             uuid PRIMARY KEY,
    user_id        uuid NOT NULL REFERENCES users(id),
    customer_email text NOT NULL,
    status         order_status NOT NULL,
    note           text,
    created_at     timestamptz NOT NULL
);

CREATE INDEX orders_user_created_idx ON orders (user_id, created_at DESC);

CREATE TABLE order_items (
    order_id   uuid NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    line_no    int NOT NULL,
    sku        text NOT NULL,
    qty        int NOT NULL CHECK (qty > 0),
    unit_price numeric(12, 2) NOT NULL CHECK (unit_price >= 0),
    PRIMARY KEY (order_id, line_no)
);
