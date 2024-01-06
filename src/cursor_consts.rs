pub const RED: &str = "\x1b[0;31;1m";
pub const GREEN: &str = "\x1b[0;32;1m";
pub const CYAN: &str = "\x1b[0;36;1m";
pub const COLOUR_RESET: &str = "\x1b[0m";
pub const CLEAR: &str = "\x1b[2J";
pub const HOME: &str = "\x1b[H";
pub const SKULL: &str = "\u{1F480}";
pub const DOORMOJI: &str = "\u{1F6AA}";

//  μαθηματικὴ τέχνη II
//
//  P(Heads) = 1/2
//  P(Heads) = 1/2
//  P(X, Y)?
//  Outcomes = [
//      (H, H), (H, T), (T, H), (T, T)
//  ]
//
//  game outcome: [w(1), w(2), w(3), ..., W(n), ..., l)
//  P(w(1), l) = 1/2
//  P(w(1) | w(2), l) = 1/3
//  P(w(1), w(2), l) = P(w(1)) * P(w(2)) * P(l(3))
//  P(S<3) = P(s=2) + P(s=1)
//  P(X, Y) = #of X,Y outcomes/#of possible outcomes
//
//  1: T            1/1     + 1 P1: 100%  | P2: 100%
//  2: F T          2/3     + 2 P1: 50%   | P2: 50%
//  3: F F T        3/6     + 3 P1: 33.3% | P2: 16.67%
//  4: F F F T      4/10    + 4 P1: 25%   | P2: 4.17%
//  5: F F F F T    5/15    + 5 P1: 20%   | P2: 0.83%
//  6: F F F F F T  6/21    + 6 P1: 16%   | P2: 0.14%
//
//  Probability of choosing correctly:
//  P1 = 1/a
//
//  Probability of clearing 2 and losing at the third:
//  P2 = P(w(n=1)) * P(w(n+1)) * P(l(n+2))
