-- Your SQL goes here

CREATE TABLE `products`
(
  `id` MEDIUMINT NOT NULL AUTO_INCREMENT,
  `name` VARCHAR(50) NOT NULL,
  `stars` DECIMAL(2, 1),
  `url` VARCHAR(255) NOT NULL,
  `price_in_idr` INT NOT NULL,

  PRIMARY KEY (id)
);
