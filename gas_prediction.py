import pandas as pd
import json
from sklearn.linear_model import QuantileRegressor
import joblib

# Load the data
with open('data.txt', 'r') as f:
    data = json.load(f)

# Convert the data to a pandas DataFrame
df = pd.json_normalize(data['blockPrices'], record_path='estimatedPrices', meta=[
                       'baseFeePerGas', 'blockNumber'])

# Convert the columns to the correct data types
for col in df.columns:
    df[col] = pd.to_numeric(df[col])

# Define the features and target
X = df[['baseFeePerGas', 'maxFeePerGas', 'maxPriorityFeePerGas', 'confidence']]
y = df['price']

# Train the quantile regression model
qr = QuantileRegressor(quantile=0.95)
qr.fit(X, y)

# Save the trained model
joblib.dump(qr, 'qr_model.joblib')


# Load the trained model
qr = joblib.load('qr_model.joblib')

# Make a prediction
X_new = [[41.017276306, 50.46, 0.85, 99]]  # Replace with your data
y_pred = qr.predict(X_new)

print(f'Predicted gas price: {y_pred[0]}')
