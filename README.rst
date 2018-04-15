Lexicon
=======

Your team's lexicon using `.toml` with keys like `Cargo.toml`.

.. code:: rust

   // print packages
   let ws = workspace::load();
   match ws.packages {
     Some(packages) => for package in packages {
       println!("{}", package);
     },
     None => (),
   }

.. code:: zsh

  % lexicon ./workspace.toml | rg '^(name|repository):|^$'
  name: Our Project Website
  repository: https://example.org/hoi/zaeme

  name: Console Application
  repository: https://example.org/hoi/zaeme


Development
-----------

.. code:: zsh

  % make help
  build       Run debug build
  clean       Spruce up
  coverage    Generate coverage report of unit tests using kcov (alias: cov)
  document    Generate documentation files (alias: doc)
  format      Check format without changes (alias: fmt)
  help        Display this message
  lint        Check code style using clippy
  test        Run unit tests and integration tests


License
-------


.. code:: text

   Lexicon
   Copyright 2018 Yasuhiro Asaka

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
