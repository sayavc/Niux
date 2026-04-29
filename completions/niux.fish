function _niux_packages
    niux -l
end

function _niux_home_packages
    niux -Hl
end

function _niux_system_packages
    niux -Sl
end

function _niux_search
    niux --search (commandline -ct)
end

complete -c niux -n "__fish_seen_subcommand_from -Hl -Hr -Hra" -f -a "(_niux_home_packages)"
complete -c niux -n "__fish_seen_subcommand_from -Sl -Sr -Sra" -f -a "(_niux_system_packages)"
complete -c niux -n "__fish_seen_subcommand_from -l" -f -a "(_niux_packages)"
complete -c niux -n "__fish_seen_subcommand_from -Hi -Si -Hia -Sia" -f -a "(_niux_search)"
