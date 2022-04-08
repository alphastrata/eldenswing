import pandas as pd
import plotly.express as px


# something like this could be cool: https://plotly.com/python/figurewidget-app/
if __name__ == "__main__":
    # timestamp,starting_souls,souls_this_run,app_yield_total,best_run,worst_run,app_startup,current_run_end_utc,current_run_start_utc,walk_one,walk_two,turn_angle
    df = pd.read_csv("../assets/history.csv", skiprows=lambda x: (x != 0) and not x % 2)

    # Delete these row indexes from dataFrame [Stupidly invalid data/initial souls count]
    # It's not possible to get this many souls in a run
    to_remove = df[df["souls_from_run"] > 99999].index
    df.drop(to_remove, inplace=True)

    # Plot them
    fig = px.histogram(df, x="souls_from_run")
    fig.show()

    fig2 = px.scatter(df, x="souls_from_run", y="walk_one")
    fig2.show()
