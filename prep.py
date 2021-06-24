
import pandas as pd
import platform

def main():
    data = pd.read_csv("results.csv")
    print(data)
    data = preprocess(data)
    data.to_csv("results_prepped.csv")

def preprocess(data: pd.DataFrame):
    data["parameterization"] = False
    data.loc[data["filename"].str.contains(
        "log|qtang|bitang"), "parameterization"] = True
    vgs = list(sorted(data.groupby("n_vertices").groups.keys()))
    if len(vgs) != 2:
        print("More than two different p's chosen done...")
        return
    data["p"] = 0.8
    data.loc[data["n_vertices"] == vgs[0], "p"] = 0.4

    uname = platform.uname()
    data["platform"] = uname.system + "---" + uname.release

    data["id"] = "Unknown"
    data.loc[data["filename"].str.contains("nm_motor.glsl"), "id"] = "motor"
    data.loc[data["filename"].str.contains("nm_rotor.glsl"), "id"] = "rotor"
    data.loc[data["filename"].str.contains("log_motor.glsl"), "id"] = "outer exponent motor"
    data.loc[data["filename"].str.contains("log_rotor.glsl"), "id"] = "outer exponent rotor"
    data.loc[data["filename"].str.contains("cayley_motor.glsl"), "id"] = "cayley motor"
    data.loc[data["filename"].str.contains("cayley_rotor.glsl"), "id"] = "cayley rotor"
    data.loc[data["filename"].str.contains("qtang.glsl"), "id"] = "qtangent rotor"
    data.loc[data["filename"].str.contains("matrix.glsl"), "id"] = "matrix"
    data.loc[data["filename"].str.contains("bitang.glsl"), "id"] = "normal and tangent"

    data = data.set_index("id")
    return data

if __name__ == "__main__":
    main()
