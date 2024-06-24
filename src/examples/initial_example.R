# Load necessary libraries
library(tidymodels)

# Set seed for reproducibility
set.seed(123)

# Split the data into training and testing sets
data_split <- initial_split(data, prop = 0.75)
train_data <- training(data_split)
test_data <- testing(data_split)

# Create a recipe for preprocessing
data_recipe <- recipe(target ~ ., data = train_data) %>%
  step_normalize(all_predictors()) %>%
  step_dummy(all_nominal_predictors())

# Print the recipe
data_recipe

# Preprocess the training data
prepped_recipe <- prep(data_recipe, training = train_data)

# Apply the recipe to the training and testing data
train_data_prepped <- bake(prepped_recipe, new_data = train_data)
test_data_prepped <- bake(prepped_recipe, new_data = test_data)

# Print preprocessed data
head(train_data_prepped)
head(test_data_prepped)
