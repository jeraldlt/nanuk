import pandas as pd
import pingouin as pn

df = pd.read_csv("../data/Weight Gain.csv")

result = pn.ttest(df["Difference"], 0.0)

result.to_csv("one_sample_t_test_weight.csv", index=False)
