# efficient-linear-regressions

Simple timing project to investigate a method to reduce the time to calculate multiple linear regressions.

This project aims to find a faster way to calculate multiple linear regressions. We'll create synthetic data, develop an optimized algorithm, measure its speed, and compare it with traditional methods.

The motivation behind this project is that in time series forecasting, we often need to calculate multiple linear regressions. For example, we may want to predict the sales of a product based on the sales of other products. We may also want to predict the sales of a product based on the sales of other products and the weather. In this case, we would need to calculate multiple linear regressions for each product. If we have 100 products, we would need to calculate 100 linear regressions. After adding in a rolling window, we would need to calculate 100 linear regressions for each time period. This can grow exponentially and be very time consuming.

By utilizing linear algebra, we can reduce the time to calculate multiple linear regressions. The goal of this project is to find out how much time we can save by using linear algebra.

## Results

The results of this project are as follows:

1. We can reduce the time to calculate multiple linear regressions by using linear algebra.
2. There is a memory tradeoff when using linear algebra.

For 10,000 regressions:

``` shell
Matrix time taken: 0.12400126457214355 seconds
Results using NumPy and linear algebra:
[[0.19120998]
 [0.44038821]
 [0.49703976]]

OLS time taken: 1.3910667896270752 seconds
Results using statsmodels:
[0.19120998 0.44038821 0.49703976]

Speed improvement: 11.218166156827834xMatrix time taken: 0.013000011444091797 seconds
Results using NumPy and linear algebra:
[[0.33865288]
 [0.6547036 ]
 [0.53215401]]

OLS time taken: 0.15204071998596191 seconds
Results using statsmodels:
[0.33865288 0.6547036  0.53215401]

Speed improvement: 11.69542970326083x
```
