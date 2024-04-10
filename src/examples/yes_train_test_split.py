import numpy as np
from sklearn.model_selection import train_test_split
from sklearn.linear_model import LinearRegression

# Generate some sample data
X = np.array([[1, 1], [2, 2], [3, 3]])
y = np.array([2, 3, 4])

# Split the data into training and testing sets
X_train, X_test, y_train, y_test = train_test_split(X, y, test_size=0.2, random_state=42)

# Fit the model
model = LinearRegression().fit(X_train, y_train)

# Make predictions
predictions = model.predict(X_test)
print(predictions)
