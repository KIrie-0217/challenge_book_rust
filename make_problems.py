import argparse
import logging
import os
import pathlib
import shutil
from typing import List

from makeToml import make_toml

_stream = logging.StreamHandler()
_stream.setLevel(logging.INFO)
_stream.setFormatter(logging.Formatter("%(asctime)s [%(levelname)s] %(filename)s %(message)s"))
logger = logging.getLogger(__name__)
logger.addHandler(_stream)
logger.setLevel(logging.DEBUG)


def main(section_name: str, problem_name: str):
    section_path: pathlib.Path = pathlib.Path("./src/").joinpath(section_name)
    if not os.path.exists(section_path):
        os.mkdir(section_path)

    problem_path: pathlib.Path = pathlib.Path("./src/").joinpath(section_name, problem_name)
    if not os.path.exists(problem_path):
        os.mkdir(problem_path)
    else:
        logger.info("section %s problem %s is already existed", section_name, problem_name)
        return

    copy_list: List[pathlib.Path] = list(pathlib.Path("./template/").glob("*"))

    print(copy_list)
    for copy_file in copy_list:
        new_file = pathlib.Path("./src/").joinpath(section_name, problem_name, copy_file.name)

        if copy_file.is_file():
            shutil.copy(copy_file, new_file)
        elif copy_file.is_dir():
            shutil.copytree(copy_file, new_file)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--section", "-s", help="section num", required=True)
    parser.add_argument("--problem", "-p", help="problem name", required=True)

    args = parser.parse_args()

    main(args.section, args.problem)
    make_toml()
