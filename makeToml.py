import glob
import shutil
from typing import List


def make_toml():
    file_list: List = []
    file_list.extend(glob.glob("./src/**/*.rs", recursive=True))
    print(file_list)

    out_file: str = "Cargo.toml"
    shutil.copy("Cargo_template.toml", out_file)

    with open(out_file, "a") as file:

        for path_name in file_list:
            if "lib" not in path_name:
                name = path_name.split("/")[-2]
                file.write("\n\n")
                file.write("[[bin]]\n")
                file.write('name = "{}"\n'.format(name))
                file.write('path = "{}"\n'.format(path_name))


if __name__ == "__main__":
    make_toml()
