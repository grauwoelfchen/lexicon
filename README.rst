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
