from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(
    name="pyston2d",
    version="0.1.0",
    rust_extensions=[RustExtension("pyston2d.pyston2d", binding=Binding.PyO3)],
    packages=["pyston2d"],
    # rust extensions are not zip safe, just like C-extensions.
    zip_safe=False,
)