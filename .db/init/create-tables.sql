-- Create shortened_urls table
CREATE TABLE IF NOT EXISTS shortened_urls (
  id varchar (12) NOT NULL,
  url varchar (250) NOT NULL,
  created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (id)
);