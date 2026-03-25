import numpy as np
from sklearn.linear_model import LinearRegression

# No train_test_split, no stratify, no pipeline
X = np.array([[1, 1], [2, 2], [3, 3]])
y = np.array([2, 3, 4])

model = LinearRegression().fit(X, y)
predictions = model.predict(X)
