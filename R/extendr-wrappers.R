# Generated by extendr: Do not edit by hand

# nolint start

#
# This file was created with the following call:
#   .Call("wrap__make_redgreen_wrappers", use_symbols = TRUE, package_name = "redgreen")

#' @usage NULL
#' @useDynLib redgreen, .registration = TRUE
NULL

#' A very simple funcion that takes in two
#' double arrays. One with the value to
#' render and another with the difference.
#' @export
redgreen <- function(value, diff) .Call(wrap__redgreen, value, diff)

#' @export
plot_values <- function(values, x, y) .Call(wrap__plot_values, values, x, y)


# nolint end
