#!/usr/bin/env perl
use strict;
use warnings;
use feature 'say';

my %nums = qw(one 1 two 2 three 3 four 4 five 5 six 6 seven 7 eight 8 nine 9);
my $restr = join "|", keys %nums;
my $total = 0;

while (<>) {
    chomp;
    print;
    print " ";
    s/($restr)/$nums{$1}/g;
    print;
    print " ";

    my ($one, $two) = /(\d)(?:.*(\d))?/;
    $two //= $one;

    say "$one$two";
    $total += "$one$two";
}

say $total;
