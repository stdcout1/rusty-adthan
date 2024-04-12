let
  pkgs = import <nixpkgs> {};
in
  pkgs.mkShell {
    buildInputs = with pkgs; [
        dbus
        webkitgtk
        alsa-lib
        openssl
        xdotool
        libappindicator-gtk3
    ];
    nativeBuildInputs = with pkgs; [
        pkg-config
    ];
    dbus = pkgs.dbus;
    shellHook = ''
        export LD_LIBRARY_PATH=${pkgs.libappindicator-gtk3}/lib
    '';
  }
