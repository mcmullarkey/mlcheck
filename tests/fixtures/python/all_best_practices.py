import numpy as np
from sklearn.model_selection import train_test_split
from sklearn.linear_model import LinearRegression
from sklearn.pipeline import Pipeline
from sklearn.preprocessing import StandardScaler

# Generate some sample data
X = np.array([[1, 1], [2, 2], [3, 3], [4, 4], [5, 5]])
y = np.array([0, 0, 1, 1, 1])

# Split with stratification and reproducibility
X_train, X_test, y_train, y_test = train_test_split(
    X, y, test_size=0.2, random_state=42, stratify=y
)

# Use a pipeline to prevent data leakage
pipe = Pipeline([
    ("scaler", StandardScaler()),
    ("model", LinearRegression()),
])

pipe.fit(X_train, y_train)
predictions = pipe.predict(X_test)
