import numpy as np
from sklearn.linear_model import LinearRegression

# Generate some sample data
X = np.array([[1, 1], [2, 2], [3, 3]])
y = np.array([2, 3, 4])

# Fit the model
model = LinearRegression().fit(X, y)

# Make predictions
predictions = model.predict(X)
print(predictions)
