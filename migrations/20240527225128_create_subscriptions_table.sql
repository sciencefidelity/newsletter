create table subscriptions(
  ID uuid NOT NULL,
  PRIMARY KEY (ID),
  email TEXT NOT NULL UNIQUE,
  name TEXT NOT NULL,
  subscribed_at timestamptz NOT NULL
);
