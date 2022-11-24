ALTER TABLE tweets ADD CONSTRAINT fk_author_id FOREIGN KEY (author_id) REFERENCES user_profile(id)
