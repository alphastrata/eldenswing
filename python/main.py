import pandas as pd
import plotly.express as px


# something like this could be cool: https://plotly.com/python/figurewidget-app/
if __name__ == "__main__":
    # timestamp,starting_souls,souls_this_run,app_yield_total,best_run,worst_run,app_startup,current_run_end_utc,current_run_start_utc,walk_one,walk_two,turn_angle
    df = pd.read_csv("assets/history.csv", skiprows=lambda x: (x != 0) and not x % 2)
    print(df.head)
    print(df.tail)

    df["souls_per_run"] = df["souls_this_run"].values - df["starting_souls"].values

    fig = px.bar(df, x="best_run", y="worst_run")

    fig.show()
