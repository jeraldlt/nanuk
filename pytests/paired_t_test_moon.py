import pandas as pd
import pingouin as pn

df = pd.read_csv("../data/Moon and Aggression.csv")

result = pn.ttest(df["Moon"], df["Other"], paired=True)

result.to_csv("paired_t_test_moon.csv", index=False)
