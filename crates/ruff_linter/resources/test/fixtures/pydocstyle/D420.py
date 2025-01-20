import warnings


def foo(x, y=1, **kwargs):
    """
    A one-line summary of the function's purpose.

    This example function contains all the sections, in the correct order,
    described by the numpy docstring standard.

    Parameters
    ----------
    x : int
        A first parameter and a super helpful description of what it is.
    y : int, optional
        The second parameter. Default is 1.
    **kwargs : dict, optional
        keyword arguments.

    Returns
    -------
    out : int
        A description of the returned value(s)

    Other Parameters
    ----------------
    multiplier : float, optional
        An additional factor to multiply the result by. Default to 1.0.

    Raises
    ------
    ValueError
        Is raised if `x` is negative.

    Warns
    -----
    UserWarning
         Warning, for example, non-critical issues.

    Warnings
    --------
    Using deprecated parameters or certain edge cases might trigger Python warnings.

    See Also
    --------
    bar : A complementary function that works closely with `foo`.

    Notes
    -----
    foo bar baz buck boy bing bling bing bang bong.

    References
    ----------
    .. [1] Doe, J. "On the use of foo in modern computing." *Journal of Examples*, 2023.
    .. [2] Roe, R. "Advanced foo techniques." *Proceedings of Foo Conference*, 2022.

    Examples
    --------
    Basic usage:

    >>> foo(2, 3)
    6
    """
    if x < 0:
        raise ValueError("Negative value provided for 'x'.")

    result = x * y * kwargs.pop('multiplier', 1.0)

    if result > 100:
        warnings.warn("Test warning. Result exceeds 100", UserWarning)

    return result



