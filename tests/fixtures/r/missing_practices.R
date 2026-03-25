# Load tidymodels
library(tidymodels)

# No set.seed, no initial_split, no strata, no recipe
model <- linear_reg() %>%
  fit(target ~ ., data = train_data)
