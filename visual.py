import pandas as pd
import matplotlib.pyplot as plt
import seaborn as sb

def main():
    data = pd.read_csv("results_prepped.csv")
    data4 = data[data["p"] == 0.4]
    data8 = data[data["p"] == 0.8]
    ax = sb.barplot(data=data.sort_values("average_drawtime"), 
                     y="id", x="average_drawtime", hue="p")
    ax.set(ylabel=None)
    ax.set(xlabel="Average Drawtime (ms)")
    ax.legend(title="Vertices per fragment", loc="upper left")
    plt.show()

if __name__ == "__main__":
    main()
