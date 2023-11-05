import time

import numpy as np
import statsmodels.api as sm

# Generate random data
num_samples = 10
num_features = 3
X = np.random.rand(num_samples, num_features)
y = np.random.rand(num_samples)

# Perform linear regressions using matrix stacking
start_time = time.time()

num_regressions = 10000
X_stacked = np.tile(X, (num_regressions, 1, 1))
print(X_stacked.shape)
y_stacked = np.tile(y, (num_regressions, 1))
print(y_stacked.shape)

print(X_stacked.transpose(0, 2, 1).shape)

results = (
    np.linalg.inv(X_stacked.transpose(0, 2, 1) @ X_stacked)
    @ X_stacked.transpose(0, 2, 1)
    @ y_stacked[..., np.newaxis]
)
matrix_time = time.time() - start_time
print("Matrix time taken: %s seconds" % (matrix_time))

print("Results using NumPy and linear algebra:")
print(results[0])

# Loop the same task using statsmodels and time it
start_time = time.time()
for i in range(num_regressions):
    model = sm.OLS(y, X)
    results = model.fit()

ols_time = time.time() - start_time
print("\nOLS time taken: %s seconds" % (ols_time))

print("Results using statsmodels:")
print(results.params)

# Calculate the speed improvement between the two methods
print("\nSpeed improvement: %sx" % (ols_time / matrix_time))
