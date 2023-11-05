# efficient-linear-regressions

Simple timing project to investigate a method to reduce the time to calculate multiple linear regressions.

This project aims to find a faster way to calculate multiple linear regressions. We'll create synthetic data, develop an optimized algorithm, measure its speed, and compare it with traditional methods.

The motivation behind this project is that in time series forecasting, we often need to calculate multiple linear regressions. For example, we may want to predict the sales of a product based on the sales of other products. We may also want to predict the sales of a product based on the sales of other products and the weather. In this case, we would need to calculate multiple linear regressions for each product. If we have 100 products, we would need to calculate 100 linear regressions. After adding in a rolling window, we would need to calculate 100 linear regressions for each time period. This can grow exponentially and be very time consuming.

By utilizing linear algebra, we can reduce the time to calculate multiple linear regressions. The goal of this project is to find out how much time we can save by using linear algebra.

## Results

The results of this project are as follows:

1. We can reduce the time to calculate multiple linear regressions by using linear algebra.
2. There is a memory tradeoff when using linear algebra.
