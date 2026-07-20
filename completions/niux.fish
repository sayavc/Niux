function _niux_packages
    niux -l --raw
end

function _niux_home_packages
    niux -Hl --raw
end

function _niux_system_packages
    niux -Sl --raw
end

function _niux_search
    niux --search (commandline -ct)
end

function _niux_has_install
    set -l cmd (commandline)
    string match -q -r -- '(-Hi|-Si|-Hia|-Sia)' "$cmd"
end

complete -c niux -n "__fish_seen_subcommand_from -l" -f -a "(_niux_packages)"
complete -c niux -n "__fish_seen_subcommand_from -Hl -Hr -Hra" -f -a "(_niux_home_packages)"
complete -c niux -n "__fish_seen_subcommand_from -Sl -Sr -Sra" -f -a "(_niux_system_packages)"
complete -c niux -n "_niux_has_install" -f -a "(_niux_search)"
