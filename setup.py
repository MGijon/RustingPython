from setuptools import find_packages, setup

with open("README.md", "r") as f:
    long_description = f.read();

setup(
    name="rust_example_package",
    version="0.0.1",
    author="Manuel Gijón Agudo",
    author_email="mgijon94@gmail.com",
    description="Collection of algorithms to test performance.",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/mgijon/RustingPython",
    install_requires=[],
    packages=find_packages(exclude=("tests",)),
    classifiers=[
        "Development Status :: 4 - Beta",
        "Programming Language :: Python :: 3",
        "Operating System :: OS Independent",
    ],
    python_requires=">=3",
    tests_require=["pytest"],
)

