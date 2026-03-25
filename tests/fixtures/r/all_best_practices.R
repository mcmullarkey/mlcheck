# Load necessary libraries
library(tidymodels)

# Set seed for reproducibility
set.seed(123)

# Split the data with stratification
data_split <- initial_split(data, prop = 0.75, strata = target)
train_data <- training(data_split)
test_data <- testing(data_split)

# Create a recipe for preprocessing
data_recipe <- recipe(target ~ ., data = train_data) %>%
  step_normalize(all_predictors()) %>%
  step_dummy(all_nominal_predictors())

prepped_recipe <- prep(data_recipe, training = train_data)
train_data_prepped <- bake(prepped_recipe, new_data = train_data)
test_data_prepped <- bake(prepped_recipe, new_data = test_data)
