import pandas as pd
import matplotlib.pyplot as plt

def main():
    data = pd.read_csv("results_prepped.csv")
    data = data.set_index("id")
    print(data)
    plt.figure()
    data["average_drawtime"].plot.bar()
    plt.show()

if __name__ == "__main__":
    main()
