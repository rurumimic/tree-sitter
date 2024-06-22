import setuptools

install_requires = [
    "tree-sitter",
]

with open("README.md", "r", encoding="utf-8", newline="\n") as fh:
    long_description = fh.read()

setuptools.setup(
    name="impala-sql-py",
    version="0.0.1",
    description="Impala SQL for Python",
    long_description=long_description,
    long_description_content_type="text/markdown",
    packages=setuptools.find_packages(),
    classifiers=[
        "Programming Language :: Python :: 3",
        "Operating System :: OS Independent",
    ],
    python_requires=">=3.10.0",
    install_requires=install_requires,
)
